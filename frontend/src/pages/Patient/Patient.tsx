// Patient.tsx
import React, { useState } from 'react';

import Details from './PatientComp/Details';
import Prescriptions from './PatientComp/Prescriptions';
import TestReports from './PatientComp/TestReports';
import Sidebar from './PatientComp/Sidebar';
import Navbar from './PatientComp/Navbar';
import Card from './PatientComp/Card'; // Import the Card component

const Patient: React.FC = () => {
  interface Prescriptions {
    title: string,
    desc: string,
    doctor: string
  }

  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const [buttonColor, setButtonColor] = useState('green');

  const toggleSidebar = () => {
    setIsSidebarOpen(!isSidebarOpen);
    setButtonColor(buttonColor === 'green' ? 'deep green' : 'green'); // Toggle button color
  };

  const prescriptions: Prescriptions[] = [
    {
      title: "Headachee",
      desc: "Maa chudao",
      doctor: 'Somya'
    },
    {
      title: "Cancer",
      desc: "Cant help",
      doctor: 'Sreyansh'
    },
    {
      title: "Depression",
      desc: "contact BC roy",
      doctor: 'Jha'
    },
    // Add more prescription data as needed
  ];

  return (
    <div>
      <div className='flex p-5 bg-green-700 text-white w-full'>
        {isSidebarOpen && <Sidebar isOpen={isSidebarOpen} toggleSidebar={toggleSidebar} />}
        <button onClick={toggleSidebar} className='rounded-full h-[60px] w-[60px] bg-green-600 mr-4'>Moulik</button>
        <Navbar />
      </div>
      <div className='flex flex-col'>
        <div className="container">
          <div className='row'>
            <div className='col-md-4'>
              <Details />
              <button className="btn-primary text-white py-2 px-4 rounded-md bg-green-700 hover:bg-green-400 focus:outline-none mr-10 ml-20">Patient List</button>
              <button className="btn-primary text-white py-2 px-4 rounded-md bg-green-700 hover:bg-green-400 focus:outline-none mr-20">Guardian List</button>
            </div>
            <div className='col-md-4'>
              <Prescriptions prescriptions={prescriptions} />
            </div>
            <div className='col-md-4'>
              <TestReports />
            </div>
          </div>
          <div className='row mt-4'>
            <div className='col-md-4 mb-2'>
              <Card icon="🏥" title="Hospital Visits" info="3 times this month" />
            </div>
            <div className='col-md-4 mb-2'>
              <Card icon="💊" title="Medications" info="Take twice daily" />
            </div>
            <div className='col-md-4 mb-2'>
              <Card icon="📅" title="Appointments" info="Scheduled for next week" />
            </div>
            <div className='col-md-4 mb-2'>
              <Card icon="🌡️" title="Temperature" info="Average temperature: 98.6°F" />
              </div>
              <div className='col-md-4 mb-2'>
              <Card icon="📋" title="Forms" info="Complete patient forms online" />
              </div>
              <div className='col-md-4 mb-2'>
              <Card icon="🧾" title="Billing" info="View and pay patient bills" />
              </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Patient;
