import { Database } from 'better-sqlite3';

export interface SyncChange {
  table: string;
  operation: 'insert' | 'update' | 'delete';
  data: Record<string, unknown>;
  recordId?: string;
}

export interface PushRequest {
  device_id: string;
  changes: SyncChange[];
  last_sync_timestamp?: string;
}

export interface SyncRecord {
  id: string;
  user_id: string;
  table_name: string;
  record_id: string;
  operation: string;
  data: string | null;
  device_id: string | null;
  timestamp: string;
  version: number;
}

export interface ConflictResult {
  resolved: SyncRecord;
  hadConflict: boolean;
}

export class SyncService {
  private db: Database;

  constructor(db: Database) {
    this.db = db;
  }

  /**
   * Process pushed changes from a client device.
   * Uses Last-Writer-Wins conflict resolution.
   */
  processPushChanges(
    userId: string,
    deviceId: string,
    changes: SyncChange[]
  ): { processed: number; conflicts: number } {
    let processed = 0;
    let conflicts = 0;

    const insertStmt = this.db.prepare(`
      INSERT INTO sync_data (id, user_id, table_name, record_id, operation, data, device_id, version)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?)
    `);

    const updateStmt = this.db.prepare(`
      UPDATE sync_data
      SET operation = ?, data = ?, device_id = ?, timestamp = datetime('now'), version = version + 1
      WHERE user_id = ? AND table_name = ? AND record_id = ?
    `);

    const existingStmt = this.db.prepare(`
      SELECT * FROM sync_data
      WHERE user_id = ? AND table_name = ? AND record_id = ?
      ORDER BY version DESC
      LIMIT 1
    `);

    const transaction = this.db.transaction(() => {
      for (const change of changes) {
        const recordId = change.recordId || (change.data?.id as string) || crypto.randomUUID();

        // Check for existing record (potential conflict)
        const existing = existingStmt.get(userId, change.table, recordId) as SyncRecord | undefined;

        if (existing) {
          // Conflict detected - resolve with Last-Writer-Wins
          conflicts++;
          const newVersion = existing.version + 1;

          updateStmt.run(
            change.operation,
            JSON.stringify(change.data),
            deviceId,
            userId,
            change.table,
            recordId
          );
        } else {
          // No conflict - insert new record
          const id = crypto.randomUUID();
          insertStmt.run(
            id,
            userId,
            change.table,
            recordId,
            change.operation,
            change.data ? JSON.stringify(change.data) : null,
            deviceId,
            1
          );
        }

        processed++;
      }

      // Update device last sync timestamp
      this.db.prepare(`
        UPDATE devices SET last_sync_at = datetime('now') WHERE id = ?
      `).run(deviceId);
    });

    transaction();

    return { processed, conflicts };
  }

  /**
   * Get changes since a given timestamp for pull sync.
   */
  getPullChanges(userId: string, sinceTimestamp: string): SyncRecord[] {
    const rows = this.db.prepare(`
      SELECT * FROM sync_data
      WHERE user_id = ? AND timestamp > ?
      ORDER BY timestamp ASC
    `).all(userId, sinceTimestamp) as SyncRecord[];

    return rows;
  }

  /**
   * Get the latest version of all records for full sync.
   */
  getFullSyncData(userId: string): SyncRecord[] {
    const rows = this.db.prepare(`
      SELECT s.* FROM sync_data s
      INNER JOIN (
        SELECT table_name, record_id, MAX(version) as max_version
        FROM sync_data
        WHERE user_id = ?
        GROUP BY table_name, record_id
      ) latest ON s.table_name = latest.table_name
        AND s.record_id = latest.record_id
        AND s.version = latest.max_version
      WHERE s.user_id = ?
      ORDER BY s.table_name, s.timestamp ASC
    `).all(userId, userId) as SyncRecord[];

    return rows;
  }

  /**
   * Get the current server timestamp.
   */
  getServerTimestamp(): string {
    const row = this.db.prepare("SELECT datetime('now') as ts").get() as { ts: string };
    return row.ts;
  }

  /**
   * Register a new device for a user.
   */
  registerDevice(userId: string, deviceName: string): string {
    const deviceId = crypto.randomUUID();

    this.db.prepare(`
      INSERT INTO devices (id, user_id, device_name, last_sync_at)
      VALUES (?, ?, ?, datetime('now'))
    `).run(deviceId, userId, deviceName);

    return deviceId;
  }

  /**
   * Get all devices for a user.
   */
  getUserDevices(userId: string): Array<{ id: string; device_name: string; last_sync_at: string | null }> {
    return this.db.prepare(`
      SELECT id, device_name, last_sync_at FROM devices WHERE user_id = ?
    `).all(userId) as Array<{ id: string; device_name: string; last_sync_at: string | null }>;
  }
}
