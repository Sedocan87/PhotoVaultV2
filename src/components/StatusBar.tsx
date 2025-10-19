import { SyncQueue } from './SyncQueue';

export function StatusBar() {
  const isBackupConnected = false;

  return (
    <footer className="p-2 border-t flex justify-between items-center">
      <div>
        <span>Backup Status: </span>
        <span className={isBackupConnected ? 'text-green-500' : 'text-red-500'}>
          {isBackupConnected ? 'Connected' : 'Disconnected'}
        </span>
      </div>
      <SyncQueue />
    </footer>
  );
}