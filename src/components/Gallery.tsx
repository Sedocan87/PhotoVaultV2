import { usePhotoStore } from '@/stores/photoStore';
import { BulkActions } from './BulkActions';

export function Gallery() {
  const { selectedFolder, selectedAlbum, selectedPhotos, togglePhotoSelection } = usePhotoStore();

  // Placeholder photos
  const photos = Array.from({ length: 12 }).map((_, index) => ({ id: index + 1 }));

  const getTitle = () => {
    if (selectedAlbum) {
      // In a real app, you would fetch the album name from the id
      return `Photos in Album ${selectedAlbum}`;
    }
    if (selectedFolder) {
      return `Photos in ${selectedFolder}`;
    }
    return 'Select a folder or album';
  };

  return (
    <main className="flex-1 p-4 flex flex-col">
      <h2 className="text-lg font-semibold mb-4">{getTitle()}</h2>
      <div className="flex-1 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
        {photos.map((photo) => (
          <div key={photo.id} className="relative">
            <div
              className={`bg-gray-200 h-48 rounded ${
                selectedPhotos.includes(photo.id) ? 'ring-2 ring-blue-500' : ''
              }`}
              onClick={() => togglePhotoSelection(photo.id)}
            ></div>
            <input
              type="checkbox"
              className="absolute top-2 left-2"
              checked={selectedPhotos.includes(photo.id)}
              onChange={() => togglePhotoSelection(photo.id)}
            />
          </div>
        ))}
      </div>
      <BulkActions />
    </main>
  );
}
