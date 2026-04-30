import { Router, Request, Response } from 'express';
import bcrypt from 'bcryptjs';
import { getDb, generateId } from '../db';
import { AuthRequest, authMiddleware, generateToken, generateRefreshToken } from '../middleware/auth';
import jwt from 'jsonwebtoken';

const router = Router();
const JWT_SECRET = process.env.JWT_SECRET || 'default-secret-change-me';

// POST /api/auth/register
router.post('/register', (req: Request, res: Response): void => {
  try {
    const { username, password } = req.body;

    if (!username || !password) {
      res.status(400).json({ error: 'Username and password are required' });
      return;
    }

    if (username.length < 3 || username.length > 50) {
      res.status(400).json({ error: 'Username must be between 3 and 50 characters' });
      return;
    }

    if (password.length < 6) {
      res.status(400).json({ error: 'Password must be at least 6 characters' });
      return;
    }

    const db = getDb();

    // Check if username already exists
    const existing = db.prepare('SELECT id FROM users WHERE username = ?').get(username);
    if (existing) {
      res.status(409).json({ error: 'Username already exists' });
      return;
    }

    // Create user
    const userId = generateId();
    const passwordHash = bcrypt.hashSync(password, 10);

    db.prepare('INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)')
      .run(userId, username, passwordHash);

    // Generate tokens
    const token = generateToken(userId, username);
    const refreshToken = generateRefreshToken(userId);

    res.status(201).json({
      message: 'User registered successfully',
      user: { id: userId, username },
      token,
      refreshToken,
    });
  } catch (err) {
    console.error('[Auth] Register error:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// POST /api/auth/login
router.post('/login', (req: Request, res: Response): void => {
  try {
    const { username, password } = req.body;

    if (!username || !password) {
      res.status(400).json({ error: 'Username and password are required' });
      return;
    }

    const db = getDb();

    const user = db.prepare('SELECT id, username, password_hash FROM users WHERE username = ?')
      .get(username) as { id: string; username: string; password_hash: string } | undefined;

    if (!user || !bcrypt.compareSync(password, user.password_hash)) {
      res.status(401).json({ error: 'Invalid username or password' });
      return;
    }

    const token = generateToken(user.id, user.username);
    const refreshToken = generateRefreshToken(user.id);

    res.json({
      message: 'Login successful',
      user: { id: user.id, username: user.username },
      token,
      refreshToken,
    });
  } catch (err) {
    console.error('[Auth] Login error:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// POST /api/auth/refresh
router.post('/refresh', (req: Request, res: Response): void => {
  try {
    const { refreshToken } = req.body;

    if (!refreshToken) {
      res.status(400).json({ error: 'Refresh token is required' });
      return;
    }

    const payload = jwt.verify(refreshToken, JWT_SECRET) as { userId: string; type: string };

    if (payload.type !== 'refresh') {
      res.status(401).json({ error: 'Invalid refresh token' });
      return;
    }

    const db = getDb();
    const user = db.prepare('SELECT id, username FROM users WHERE id = ?')
      .get(payload.userId) as { id: string; username: string } | undefined;

    if (!user) {
      res.status(401).json({ error: 'User not found' });
      return;
    }

    const newToken = generateToken(user.id, user.username);
    const newRefreshToken = generateRefreshToken(user.id);

    res.json({
      token: newToken,
      refreshToken: newRefreshToken,
    });
  } catch (err) {
    if (err instanceof jwt.TokenExpiredError) {
      res.status(401).json({ error: 'Refresh token expired', code: 'REFRESH_TOKEN_EXPIRED' });
    } else {
      res.status(401).json({ error: 'Invalid refresh token' });
    }
  }
});

// GET /api/auth/me (protected)
router.get('/me', authMiddleware, (req: AuthRequest, res: Response): void => {
  try {
    const db = getDb();
    const user = db.prepare('SELECT id, username, created_at FROM users WHERE id = ?')
      .get(req.userId) as { id: string; username: string; created_at: string } | undefined;

    if (!user) {
      res.status(404).json({ error: 'User not found' });
      return;
    }

    res.json({ user });
  } catch (err) {
    console.error('[Auth] Get user error:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
});

export default router;
