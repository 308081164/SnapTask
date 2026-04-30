import path from 'path';
import dotenv from 'dotenv';
import express from 'express';
import cors from 'cors';
import helmet from 'helmet';
import compression from 'compression';
import { initDatabase, getDb } from './db';
import authRoutes from './routes/auth';
import syncRoutes from './routes/sync';

// Load environment variables
dotenv.config();

const PORT = parseInt(process.env.PORT || '3000', 10);
const CORS_ORIGIN = process.env.CORS_ORIGIN || '*';

async function bootstrap() {
  const app = express();

  // Initialize database
  initDatabase();
  console.log('[SnapTask] Database initialized successfully');

  // Middleware
  app.use(helmet());
  app.use(compression());
  app.use(cors({
    origin: CORS_ORIGIN === '*' ? true : CORS_ORIGIN.split(',').map(s => s.trim()),
    credentials: true,
  }));
  app.use(express.json({ limit: '10mb' }));

  // Request logging
  app.use((req, _res, next) => {
    console.log(`[${new Date().toISOString()}] ${req.method} ${req.path}`);
    next();
  });

  // Health check
  app.get('/api/health', (_req, res) => {
    const db = getDb();
    const row = db.prepare('SELECT sqlite_version() as version').get() as { version: string };
    res.json({
      status: 'ok',
      timestamp: new Date().toISOString(),
      sqlite_version: row.version,
    });
  });

  // Routes
  app.use('/api/auth', authRoutes);
  app.use('/api/sync', syncRoutes);

  // 404 handler
  app.use((_req, res) => {
    res.status(404).json({ error: 'Not Found' });
  });

  // Global error handler
  app.use((err: Error, _req: express.Request, res: express.Response, _next: express.NextFunction) => {
    console.error('[SnapTask] Unhandled error:', err.message);
    res.status(500).json({ error: 'Internal Server Error' });
  });

  // Start server
  app.listen(PORT, () => {
    console.log(`[SnapTask] Server running on http://localhost:${PORT}`);
    console.log(`[SnapTask] CORS origin: ${CORS_ORIGIN}`);
  });
}

bootstrap().catch((err) => {
  console.error('[SnapTask] Failed to start server:', err);
  process.exit(1);
});

export { app };
