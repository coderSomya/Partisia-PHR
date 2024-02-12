// Sidebar.tsx
import React from 'react';

interface SidebarProps {
  isOpen: boolean;
  toggleSidebar: () => void;
}

const Sidebar: React.FC<SidebarProps> = ({ isOpen, toggleSidebar }) => {
  return (
    <div className={`sidebar ${isOpen ? 'open' : ''}`} style={{ backgroundColor: 'deep green', margin: '20px' }}>
      <ul style={{ listStyle: 'none', padding: 0 }}>
        <li style={{ backgroundColor: 'lightgreen', padding: '10px', marginBottom: '5px', borderRadius: '5px' }}>Option 1</li>
        <li style={{ backgroundColor: 'lightgreen', padding: '10px', marginBottom: '5px', borderRadius: '5px' }}>Option 2</li>
        <li style={{ backgroundColor: 'lightgreen', padding: '10px', marginBottom: '5px', borderRadius: '5px' }}>Option 3</li>
      </ul>
      <button onClick={toggleSidebar} className="btn btn-primary toggle-btn" style={{ padding: '10px', backgroundColor: 'lightgreen', border: 'none', borderRadius: '5px', cursor: 'pointer' }}>Close</button>
    </div>
  );
};




export default Sidebar;
