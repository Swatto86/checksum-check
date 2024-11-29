import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

function App() {
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [checksum, setChecksum] = useState<string>('');
  const [algorithm, setAlgorithm] = useState<string>('SHA-256');
  const [theme, setTheme] = useState('light');

  const handleFileSelect = async () => {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'All Files',
        extensions: ['*']
      }]
    });
    
    if (selected && !Array.isArray(selected)) {
      setSelectedFile(selected);
      try {
        const result = await invoke('calculate_checksum', { 
          filePath: selected,
          algorithm: algorithm.toLowerCase()
        });
        setChecksum(result as string);
      } catch (error) {
        console.error('Error calculating checksum:', error);
      }
    }
  };

  const toggleTheme = () => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
  };

  useEffect(() => {
    // Set initial theme
    document.documentElement.setAttribute('data-theme', theme);
  }, []);

  return (
    <div className="min-h-screen">
      {/* Theme toggle button */}
      <button 
        className="btn btn-circle swap swap-rotate fixed top-4 right-4"
        onClick={toggleTheme}
      >
        {/* sun icon */}
        <svg className={`w-6 h-6 ${theme === 'dark' ? 'block' : 'hidden'}`} fill="currentColor" viewBox="0 0 20 20">
          <path fillRule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clipRule="evenodd" />
        </svg>
        {/* moon icon */}
        <svg className={`w-6 h-6 ${theme === 'light' ? 'block' : 'hidden'}`} fill="currentColor" viewBox="0 0 20 20">
          <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
        </svg>
      </button>
      
      <div className="min-h-screen bg-base-100 p-4">
        <div className="max-w-xl mx-auto space-y-4">
          <h1 className="text-2xl font-bold text-center mb-8">Checksum Checker</h1>
          
          <div className="form-control">
            <label className="label">
              <span className="label-text">Hash Algorithm</span>
            </label>
            <select 
              className="select select-bordered w-full"
              value={algorithm}
              onChange={(e) => setAlgorithm(e.target.value)}
            >
              <option>MD5</option>
              <option>SHA-1</option>
              <option>SHA-256</option>
              <option>SHA-512</option>
            </select>
          </div>

          <button 
            className="btn btn-primary w-full"
            onClick={handleFileSelect}
          >
            Select File
          </button>

          {selectedFile && (
            <div className="card bg-base-200 p-4">
              <h3 className="font-semibold">Selected File:</h3>
              <p className="text-sm break-all">{selectedFile}</p>
            </div>
          )}

          {checksum && (
            <div className="card bg-base-200 p-4">
              <h3 className="font-semibold">{algorithm} Checksum:</h3>
              <p className="text-sm break-all font-mono">{checksum}</p>
              <button 
                className="btn btn-sm btn-ghost mt-2"
                onClick={() => navigator.clipboard.writeText(checksum)}
              >
                Copy to Clipboard
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App; 