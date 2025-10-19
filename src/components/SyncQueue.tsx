import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface QueueStatus {
  pending_operations: number;
}

export function SyncQueue() {
  const [status, setStatus] = useState<QueueStatus>({ pending_operations: 0 });

  useEffect(() => {
    const interval = setInterval(() => {
      invoke<QueueStatus>('get_sync_queue_status')
        .then(setStatus)
        .catch(console.error);
    }, 2000); // Poll every 2 seconds

    return () => clearInterval(interval);
  }, []);

  return (
    <div>
      <span>Pending Operations: {status.pending_operations}</span>
    </div>
  );
}
