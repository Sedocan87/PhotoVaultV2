import {create} from 'zustand';

interface PhotoState {
  selectedFolder: string | null;
  setSelectedFolder: (folder: string | null) => void;
  selectedAlbum: number | null;
  setSelectedAlbum: (albumId: number | null) => void;
  selectedPhotos: number[];
  setSelectedPhotos: (photos: number[]) => void;
  togglePhotoSelection: (photoId: number) => void;
}

export const usePhotoStore = create<PhotoState>((set) => ({
  selectedFolder: null,
  setSelectedFolder: (folder) => set({ selectedFolder: folder, selectedAlbum: null }),
  selectedAlbum: null,
  setSelectedAlbum: (albumId) => set({ selectedAlbum: albumId, selectedFolder: null }),
  selectedPhotos: [],
  setSelectedPhotos: (photos) => set({ selectedPhotos: photos }),
  togglePhotoSelection: (photoId) =>
    set((state) => {
      const selectedPhotos = state.selectedPhotos.includes(photoId)
        ? state.selectedPhotos.filter((id) => id !== photoId)
        : [...state.selectedPhotos, photoId];
      return { selectedPhotos };
    }),
}));
