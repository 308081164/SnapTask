import path from 'path';
import fs from 'fs';
import Database from 'better-sqlite3';
import { v4 as uuidv4 } from 'uuid';

let db: Database.Database;

export function initDatabase(): void {
  const dbPath = process.env.DATABASE_PATH || './data/snaptask.db';
  const dbDir = path.dirname(dbPath);

  // Ensure database directory exists
  if (!fs.existsSync(dbDir)) {
    fs.mkdirSync(dbDir, { recursive: true });
  }

  db = new Database(dbPath);

  // Enable WAL mode for better concurrent read performance
  db.pragma('journal_mode = WAL');
  db.pragma('foreign_keys = ON');

  // Create tables
  createTables();

  console.log(`[DB] Database opened: ${dbPath}`);
}

function createTables(): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS users (
      id TEXT PRIMARY KEY,
      username TEXT NOT NULL UNIQUE,
      password_hash TEXT NOT NULL,
      created_at TEXT NOT NULL DEFAULT (datetime('now'))
    );

    CREATE TABLE IF NOT EXISTS devices (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL,
      device_name TEXT NOT NULL,
      last_sync_at TEXT,
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS sync_data (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL,
      table_name TEXT NOT NULL,
      record_id TEXT NOT NULL,
      operation TEXT NOT NULL CHECK(operation IN ('insert', 'update', 'delete')),
      data TEXT,
      device_id TEXT,
      timestamp TEXT NOT NULL DEFAULT (datetime('now')),
      version INTEGER NOT NULL DEFAULT 1,
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
      FOREIGN KEY (device_id) REFERENCES devices(id) ON DELETE SET NULL
    );

    CREATE INDEX IF NOT EXISTS idx_sync_data_user_timestamp
      ON sync_data(user_id, timestamp);

    CREATE INDEX IF NOT EXISTS idx_sync_data_user_table_record
      ON sync_data(user_id, table_name, record_id);

    CREATE INDEX IF NOT EXISTS idx_devices_user
      ON devices(user_id);
  `);

  console.log('[DB] Tables created/verified successfully');
}

export function getDb(): Database.Database {
  if (!db) {
    throw new Error('Database not initialized. Call initDatabase() first.');
  }
  return db;
}

export function generateId(): string {
  return uuidv4();
}
