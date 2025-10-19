import { useEffect } from 'react';
import toast from 'react-hot-toast';
import { listen } from '@tauri-apps/api/event';

export function useSyncEvents() {
  useEffect(() => {
    const unlisten = listen('sync-event', (event) => {
      console.log('Sync event:', event.payload);
      toast.success('Sync event received!');
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);
}
