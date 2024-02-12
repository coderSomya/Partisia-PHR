import React from 'react';
import Details from './PatientComp/Details';
import Prescriptions from './PatientComp/Prescriptions';
import TestReports from './PatientComp/TestReports';
import Sidebar from './PatientComp/Sidebar';

const Patient : React.FC = () => {
  interface Prescriptions {
    title: string,
    desc: string,
    doctor: string
  }

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
       }
  ];

  return (
    <div>
      <Sidebar />
      <div className='flex flex-col'>
        <div className='flex p-5 shadow:lg w-full h-[200px] bg-green-400 text-white bg-green'>
          <div className='rounded-full h-[60px] w-[60px]'> dp</div>
          <div className='flex-1 text-3xl'>Moulik</div>
          <div className='bg-green-600 rounded-lg text-white p-3 text-xl cursor-pointer'>Request prescription</div>
        </div>
        <div className="container">
          <div className='row'>
            <div className='col-md-4'>
              <Details />
            </div>
            <div className='col-md-4'>
              <Prescriptions prescriptions={prescriptions} />
            </div>
            <div className='col-md-4'>
              <TestReports />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Patient;
