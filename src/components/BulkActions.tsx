import { usePhotoStore } from '@/stores/photoStore';
import { invoke } from '@tauri-apps/api/tauri';

export function BulkActions() {
  const { selectedPhotos, setSelectedPhotos } = usePhotoStore();

  if (selectedPhotos.length === 0) {
    return null;
  }

  const handleDelete = () => {
    invoke('delete_photos', { photoIds: selectedPhotos })
      .then(() => {
        setSelectedPhotos([]);
      })
      .catch(console.error);
  };

  return (
    <div className="p-4 bg-gray-100 border-t">
      <h3 className="text-lg font-semibold mb-2">{selectedPhotos.length} photos selected</h3>
      <button
        className="p-2 bg-red-500 text-white rounded mr-2"
        onClick={handleDelete}
      >
        Delete
      </button>
      {/* Add other bulk actions here */}
    </div>
  );
}
