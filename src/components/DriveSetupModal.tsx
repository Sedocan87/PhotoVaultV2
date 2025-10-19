import { useState } from 'react';

export function DriveSetupModal({
  onClose,
  onSave,
}: {
  onClose: () => void;
  onSave: (primary: string, backup: string) => void;
}) {
  const [primary, setPrimary] = useState('');
  const [backup, setBackup] = useState('');

  const handleSave = () => {
    onSave(primary, backup);
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
      <div className="bg-white p-8 rounded-lg">
        <h2 className="text-lg font-semibold mb-4">Setup Drives</h2>
        <div className="mb-4">
          <label className="block mb-2">Primary Drive</label>
          <input
            type="text"
            className="w-full p-2 border rounded"
            value={primary}
            onChange={(e) => setPrimary(e.target.value)}
          />
        </div>
        <div className="mb-4">
          <label className="block mb-2">Backup Drive</label>
          <input
            type="text"
            className="w-full p-2 border rounded"
            value={backup}
            onChange={(e) => setBackup(e.target.value)}
          />
        </div>
        <div className="flex justify-end">
          <button className="p-2 mr-2" onClick={onClose}>
            Cancel
          </button>
          <button className="p-2 bg-blue-500 text-white rounded" onClick={handleSave}>
            Save
          </button>
        </div>
      </div>
    </div>
  );
}
