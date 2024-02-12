import React from 'react';

const MedicalTestReports: React.FC = () => {
  return (
    <div className='p-3 mb-3 shadow bg-light'>
      <h1>Medical Test Reports</h1>

      {/* Blood Test Report */}
      <div className='card mb-3'>
        <div className='card-body'>
          <h5 className='card-title'>Blood Test Report</h5>
          <p className='card-text'>Summary of the blood test report findings...</p>
          <button type='button' className='btn btn-success'>View Report</button>
        </div>
      </div>

      {/* Urine Test Report */}
      <div className='card mb-3'>
        <div className='card-body'>
          <h5 className='card-title'>Urine Test Report</h5>
          <p className='card-text'>Summary of the urine test report findings...</p>
          <button type='button' className='btn btn-success'>View Report</button>
        </div>
      </div>

      {/* Imaging Test Report */}
      <div className='card mb-3'>
        <div className='card-body'>
          <h5 className='card-title'>Imaging Test Report</h5>
          <p className='card-text'>Summary of the imaging test report findings...</p>
          <button type='button' className='btn btn-success'>View Report</button>
        </div>
      </div>
    </div>
  );
};

export default MedicalTestReports;

