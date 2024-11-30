import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Window, LogicalSize } from '@tauri-apps/api/window';

function App() {
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [hashResults, setHashResults] = useState<{
    md5: string;
    sha1: string;
    sha256: string;
    sha512: string;
  } | null>(null);
  const [theme, setTheme] = useState('dark');
  const [isDragging, setIsDragging] = useState(false);
  const [copiedHash, setCopiedHash] = useState<string | null>(null);

  useEffect(() => {
    if (hashResults) {
      const resizeWindow = async () => {
        const appWindow = new Window('main');
        const documentHeight = document.documentElement.scrollHeight;
        await appWindow.setSize(new LogicalSize(800, documentHeight + 40));
      };
      resizeWindow();
    }
  }, [hashResults]);

  const handleFileSelect = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'All Files',
          extensions: ['*']
        }]
      });
    
      if (selected === null) {
        return;
      }

      const filePath = Array.isArray(selected) ? selected[0] : selected;
      setSelectedFile(filePath);
      
      try {
        const result = await invoke('calculate_checksum', { path: filePath });
        setHashResults(result as any);
      } catch (error) {
        console.error('Error calculating checksum:', error);
      }
    } catch (error) {
      console.error('Error selecting file:', error);
    }
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);

    const file = e.dataTransfer.files[0];
    if (file) {
      const filePath = await new Promise<string>((resolve) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result as string);
        reader.readAsDataURL(file);
      });
      setSelectedFile(filePath);
      try {
        const result = await invoke('calculate_checksum', { path: filePath });
        setHashResults(result as any);
      } catch (error) {
        console.error('Error calculating checksum:', error);
      }
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
  };

  const toggleTheme = () => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
  };

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, []);

  return (
    <div className="min-h-screen p-8">
      <button 
        className="btn btn-circle swap swap-rotate fixed top-4 right-4"
        onClick={toggleTheme}
      >
        <svg className={`w-6 h-6 ${theme === 'dark' ? 'block' : 'hidden'}`} fill="currentColor" viewBox="0 0 20 20">
          <path fillRule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clipRule="evenodd" />
        </svg>
        <svg className={`w-6 h-6 ${theme === 'light' ? 'block' : 'hidden'}`} fill="currentColor" viewBox="0 0 20 20">
          <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
        </svg>
      </button>

      <div className="max-w-xl mx-auto space-y-4">
        <h1 className="text-4xl font-bold text-center mb-8 bg-gradient-to-r from-blue-500 to-purple-600 bg-clip-text text-transparent"># Checksum Check</h1>
        
        <p className="text-center text-gray-400 mb-4">Select or drop a file to calculate its checksums</p>
        <p className="text-center text-gray-500 text-sm mb-8">Supports MD5, SHA1, SHA256, and SHA512</p>

        <div 
          className={`border-2 border-dashed rounded-lg p-8 text-center cursor-pointer hover:border-primary transition-colors ${isDragging ? 'border-primary bg-base-200' : ''}`}
          onClick={handleFileSelect}
          onDrop={handleDrop}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
        >
          <div className="flex flex-col items-center justify-center">
            <svg className="w-12 h-12 mb-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
            </svg>
            <p className="text-gray-400">Drop your file here</p>
            <p className="text-gray-500 text-sm mt-2">or click to browse</p>
          </div>
        </div>

        {selectedFile && (
          <div className="space-y-4">
            <div className="card bg-base-200 p-4">
              <h3 className="font-semibold mb-2">File Information</h3>
              <p className="text-sm break-all">Path: {selectedFile}</p>
            </div>

            {hashResults && (
              <div className="card bg-base-200 p-4">
                <h3 className="font-semibold mb-4">Checksums</h3>
                <div className="space-y-4">
                  {Object.entries(hashResults).map(([algo, hash]) => (
                    <div key={algo} className="space-y-1">
                      <div className="flex justify-between items-center">
                        <span className="text-sm font-medium uppercase">{algo}:</span>
                        <button 
                          className={`btn btn-xs ${copiedHash === algo ? 'btn-success' : 'btn-ghost'}`}
                          onClick={() => {
                            navigator.clipboard.writeText(hash);
                            setCopiedHash(algo);
                            setTimeout(() => setCopiedHash(null), 1000);
                          }}
                        >
                          {copiedHash === algo ? 'Copied!' : 'Copy'}
                        </button>
                      </div>
                      <p className="text-sm break-all font-mono bg-base-300 p-2 rounded">{hash}</p>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
      
      <p className="text-center text-gray-500 text-xs mt-8">All calculations are performed locally on your device</p>
    </div>
  );
}

export default App; 