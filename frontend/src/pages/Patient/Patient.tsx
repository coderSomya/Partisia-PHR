import React from 'react'

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

 ]
  return (
    <div>

        <div className='flex flex-col'>
            <div className='flex p-5 shadow:lg w-full h-[200px] bg-green-400 text-white bg-green'>
                <div className='rounded-full h-[60px] w-[60px]'> dp</div>
                <div className='flex-1 text-3xl'>Moulik</div>
                <div className='bg-green-600 rounded-lg text-white p-3 text-xl cursor-pointer'>Request prescription</div>
            </div>
            <div className='flex flex-row'>
                <div className='w-1/3 flex flex-col'>Details
                <div className='flex flex-col p-3'>
                    <ul>
                        <li className='text-2xl p-3 m-2'>height: 160cm</li>
                        <li className='text-2xl p-3 m-2'>width: 70kg</li>
                        <li className='text-2xl p-3 m-2'>Blood pressure: 760 mm of Hg</li>
                    </ul>
                </div>
                </div>
                <div className='w-2/3 h-screen overflow-y-auto flex flex-col'> Prescriptions
                {
                    prescriptions.map((prescription: Prescriptions)=>{
                    return(<>
                    <div className='w-full flex flex-col p-3 mx-4 shadow-lg cursor-pointer bg-gray text-black'>
                       <div className='flex flex-row items-center'>
                         <div className='text-3xl'>{prescription.title}</div>
                         <div className='m-2 p-3'> {prescription.doctor}</div>
                       </div>
                       <br />
                       <div className='flex flex-row justify-start'>
                     <p >{prescription.desc}</p>
                     </div>
                    </div>
                    </>)
 } )
                }
                </div>
                
            </div>
        </div>
    </div>
  )
}

export default Patient