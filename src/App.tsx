import { useState } from 'react';
import { Library } from './pages/Library';
import { Settings } from './pages/Settings';
import { StatusBar } from './components/StatusBar';
import { Toaster } from 'react-hot-toast';
import { useSyncEvents } from './hooks/useSyncEvents';

function App() {
  const [page, setPage] = useState('library');
  useSyncEvents();

  return (
    <div className="App flex flex-col h-screen">
      <Toaster />
      <header className="p-4 border-b">
        <nav>
          <button className="mr-4" onClick={() => setPage('library')}>
            Library
          </button>
          <button onClick={() => setPage('settings')}>Settings</button>
        </nav>
      </header>
      <main className="flex-1 overflow-y-auto">
        {page === 'library' && <Library />}
        {page === 'settings' && <Settings />}
      </main>
      <StatusBar />
    </div>
  );
}

export default App;