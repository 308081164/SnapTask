import { Router, Response } from 'express';
import { getDb } from '../db';
import { AuthRequest, authMiddleware } from '../middleware/auth';
import { SyncService } from '../services/syncService';

const router = Router();

// POST /api/sync/push - Push changes from client
router.post('/push', authMiddleware, (req: AuthRequest, res: Response): void => {
  try {
    const { device_id, changes, last_sync_timestamp } = req.body;

    if (!device_id || !changes || !Array.isArray(changes)) {
      res.status(400).json({ error: 'device_id and changes array are required' });
      return;
    }

    if (changes.length === 0) {
      const db = getDb();
      const service = new SyncService(db);
      const serverTimestamp = service.getServerTimestamp();
      res.json({ success: true, server_timestamp: serverTimestamp, processed: 0, conflicts: 0 });
      return;
    }

    // Validate device belongs to user
    const db = getDb();
    const device = db.prepare('SELECT id FROM devices WHERE id = ? AND user_id = ?')
      .get(device_id, req.userId);

    if (!device) {
      res.status(403).json({ error: 'Device not registered for this user' });
      return;
    }

    const service = new SyncService(db);
    const result = service.processPushChanges(req.userId!, device_id, changes);
    const serverTimestamp = service.getServerTimestamp();

    res.json({
      success: true,
      server_timestamp: serverTimestamp,
      processed: result.processed,
      conflicts: result.conflicts,
    });
  } catch (err) {
    console.error('[Sync] Push error:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// GET /api/sync/pull?since=<timestamp> - Pull changes since timestamp
router.get('/pull', authMiddleware, (req: AuthRequest, res: Response): void => {
  try {
    const since = req.query.since as string;

    if (!since) {
      res.status(400).json({ error: 'since query parameter is required' });
      return;
    }

    const db = getDb();
    const service = new SyncService(db);
    const changes = service.getPullChanges(req.userId!, since);
    const serverTimestamp = service.getServerTimestamp();

    // Parse JSON data for each change
    const parsedChanges = changes.map(change => ({
      id: change.id,
      table: change.table_name,
      recordId: change.record_id,
      operation: change.operation,
      data: change.data ? JSON.parse(change.data) : null,
      deviceId: change.device_id,
      timestamp: change.timestamp,
      version: change.version,
    }));

    res.json({
      server_timestamp: serverTimestamp,
      changes: parsedChanges,
    });
  } catch (err) {
    console.error('[Sync] Pull error:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// GET /api/sync/full - Full sync (all current data)
router.get('/full', authMiddleware, (req: AuthRequest, res: Response): void => {
  try {
    const db = getDb();
    const service = new SyncService(db);
    const records = service.getFullSyncData(req.userId!);
    const serverTimestamp = service.getServerTimestamp();

    // Parse JSON data for each record
    const parsedRecords = records.map(record => ({
      id: record.id,
      table: record.table_name,
      recordId: record.record_id,
      operation: record.operation,
      data: record.data ? JSON.parse(record.data) : null,
      deviceId: record.device_id,
      timestamp: record.timestamp,
      version: record.version,
    }));

    res.json({
      server_timestamp: serverTimestamp,
      changes: parsedRecords,
    });
  } catch (err) {
    console.error('[Sync] Full sync error:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// POST /api/sync/register-device - Register a new device
router.post('/register-device', authMiddleware, (req: AuthRequest, res: Response): void => {
  try {
    const { device_name } = req.body;

    if (!device_name) {
      res.status(400).json({ error: 'device_name is required' });
      return;
    }

    const db = getDb();
    const service = new SyncService(db);
    const deviceId = service.registerDevice(req.userId!, device_name);

    res.status(201).json({
      device_id: deviceId,
      device_name,
    });
  } catch (err) {
    console.error('[Sync] Register device error:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// GET /api/sync/devices - List user's devices
router.get('/devices', authMiddleware, (req: AuthRequest, res: Response): void => {
  try {
    const db = getDb();
    const service = new SyncService(db);
    const devices = service.getUserDevices(req.userId!);

    res.json({ devices });
  } catch (err) {
    console.error('[Sync] Get devices error:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
});

export default router;
