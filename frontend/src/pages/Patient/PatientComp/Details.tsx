import React from 'react';

const Details: React.FC = () => {
  return (
    <div className='p-3 mb-3 shadow bg-light'>
      <h1 style={{ display: 'block' }}>Details</h1>
      <ul className='list-group'>
        <li className='list-group-item'>Height: 160cm</li>
        <li className='list-group-item'>Weight: 70kg</li>
        <li className='list-group-item'>Blood Pressure: 760 mm of Hg</li>
      </ul>
    </div>
  );
};

export default Details;

