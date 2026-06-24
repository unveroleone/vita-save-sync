import fs from 'fs';
import path from 'path';

export function dataDir(): string {
  return process.env.DATA_DIR ?? path.join(process.cwd(), 'data');
}

export function userDir(userName: string): string {
  return path.join(dataDir(), 'users', userName);
}

export function savesDir(userName: string): string {
  return path.join(userDir(userName), 'saves');
}

export function titleDir(userName: string, titleId: string): string {
  return path.join(savesDir(userName), titleId);
}

export function manifestPath(userName: string): string {
  return path.join(userDir(userName), 'manifest.json');
}

export function devicesDir(userName: string): string {
  return path.join(userDir(userName), 'devices');
}

export function ensureDir(dir: string): void {
  fs.mkdirSync(dir, { recursive: true });
}

export interface GameEntry {
  title?: string;
  latestVersion: string;
  latestHash: string;
  uploadedBy: string;
  size: number;
  versionCount?: number;
}

export function countVersions(userName: string, titleId: string): number {
  const versionsPath = path.join(titleDir(userName, titleId), 'versions');
  if (!fs.existsSync(versionsPath)) return 0;
  return fs.readdirSync(versionsPath).filter(f => f.endsWith('.zip')).length;
}

export interface Manifest {
  userId: string;
  updatedAt: string;
  games: Record<string, GameEntry>;
}

export function readManifest(userName: string): Manifest {
  const mp = manifestPath(userName);
  if (!fs.existsSync(mp)) {
    return { userId: userName, updatedAt: new Date().toISOString(), games: {} };
  }
  return JSON.parse(fs.readFileSync(mp, 'utf8')) as Manifest;
}

export function writeManifest(userName: string, manifest: Manifest): void {
  ensureDir(userDir(userName));
  fs.writeFileSync(manifestPath(userName), JSON.stringify(manifest, null, 2));
}
