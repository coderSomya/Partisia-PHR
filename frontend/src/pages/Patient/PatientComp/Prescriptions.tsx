import React from 'react';

interface Prescription {
  title: string;
  desc: string;
  doctor: string;
}

interface Props {
  prescriptions: Prescription[];
}

const Prescriptions: React.FC<Props> = ({ prescriptions }) => {
  return (
    <div className='p-3 mb-3 shadow bg-light'>
      <h1>Prescriptions</h1>
      {prescriptions.map((prescription, index) => (
        <div key={index} className='card mb-3'>
          <div className='card-body'>
            <h5 className='card-title'>{prescription.title}</h5>
            <p className='card-text'>{prescription.desc}</p>
            <p className='card-text'><small className='text-muted'>Doctor: {prescription.doctor}</small></p>
          </div>
        </div>
      ))}
    </div>
  );
};

export default Prescriptions;
