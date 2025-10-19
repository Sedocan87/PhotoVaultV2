import { useState } from 'react';

export function Settings() {
  const [drives, setDrives] = useState({ primary: '', backup: '' });

  const handleSaveDrives = (primary: string, backup: string) => {
    setDrives({ primary, backup });
    // Here you would typically call a Tauri command to save the config
    console.log('Saving drives:', { primary, backup });
  };

  return (
    <div className="p-8">
      <h1 className="text-2xl font-bold mb-8">Settings</h1>

      {/* Drive Configuration */}
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Drive Configuration</h2>
        <div className="space-y-2">
          <p><span className="font-semibold">Primary Drive:</span> {drives.primary || 'Not set'}</p>
          <p><span className="font-semibold">Backup Drive:</span> {drives.backup || 'Not set'}</p>
        </div>
        <button
          className="mt-4 p-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
          onClick={() => { /* Logic to open drive setup */ }}
        >
          Change Drives
        </button>
      </div>

      {/* App Preferences */}
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">App Preferences</h2>
        <div className="space-y-4">
          <div>
            <label htmlFor="theme-select" className="block font-semibold mb-2">Theme</label>
            <select id="theme-select" className="p-2 border rounded bg-gray-800 text-white">
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </div>
          {/* Add more preferences here, e.g., startup behavior */}
        </div>
      </div>

      {/* About Section */}
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">About</h2>
        <p>PhotoVault Version 1.0.0</p>
        {/* Add links to documentation, license, etc. */}
      </div>

      {/* Data Management */}
      <div>
        <h2 className="text-xl font-semibold mb-4">Data Management</h2>
        <button
          className="p-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
          onClick={() => { /* Logic to clear cache */ }}
        >
          Clear Cache & Rescan
        </button>
        <p className="text-sm text-gray-400 mt-2">This will clear all cached data and rescan your library. Use with caution.</p>
      </div>
    </div>
  );
}
