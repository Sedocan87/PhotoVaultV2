import { Sidebar } from '@/components/Sidebar';
import { Gallery } from '@/components/Gallery';

export function Library() {
  return (
    <div className="flex h-screen">
      <Sidebar />
      <Gallery />
    </div>
  );
}
