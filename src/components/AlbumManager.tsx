import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { usePhotoStore } from '@/stores/photoStore';

interface Album {
  id: number;
  name: string;
  created_at: string;
}

export function AlbumManager() {
  const { selectedAlbum, setSelectedAlbum } = usePhotoStore();
  const [albums, setAlbums] = useState<Album[]>([]);
  const [newAlbumName, setNewAlbumName] = useState('');

  const fetchAlbums = () => {
    invoke<Album[]>('get_albums')
      .then(setAlbums)
      .catch(console.error);
  };

  const handleCreateAlbum = () => {
    invoke('create_album', { name: newAlbumName })
      .then(() => {
        setNewAlbumName('');
        fetchAlbums();
      })
      .catch(console.error);
  };

  useEffect(() => {
    fetchAlbums();
  }, []);

  return (
    <div className="p-4">
      <h2 className="text-lg font-semibold mb-4">Albums</h2>
      <div className="flex mb-4">
        <input
          type="text"
          className="w-full p-2 border rounded mr-2"
          value={newAlbumName}
          onChange={(e) => setNewAlbumName(e.target.value)}
          placeholder="New album name"
        />
        <button
          className="p-2 bg-blue-500 text-white rounded"
          onClick={handleCreateAlbum}
        >
          Create
        </button>
      </div>
      <ul>
        {albums.map((album) => (
          <li key={album.id}>
            <button
              className={`w-full text-left p-2 rounded ${
                selectedAlbum === album.id ? 'bg-gray-200' : ''
              }`}
              onClick={() => setSelectedAlbum(album.id)}
            >
              {album.name}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}