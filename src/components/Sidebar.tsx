import { usePhotoStore } from '@/stores/photoStore';
import { AlbumManager } from './AlbumManager';

const folders = ['Folder 1', 'Folder 2', 'Folder 3'];

export function Sidebar() {
  const { selectedFolder, setSelectedFolder } = usePhotoStore();

  return (
    <aside className="w-64 p-4 border-r">
      <h2 className="text-lg font-semibold mb-4">Folders</h2>
      <ul>
        {folders.map((folder) => (
          <li key={folder}>
            <button
              className={`w-full text-left p-2 rounded ${
                selectedFolder === folder ? 'bg-gray-200' : ''
              }`}
              onClick={() => setSelectedFolder(folder)}
            >
              {folder}
            </button>
          </li>
        ))}
      </ul>
      <hr className="my-4" />
      <AlbumManager />
    </aside>
  );
}