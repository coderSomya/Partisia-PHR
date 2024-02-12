import { useNavigate } from 'react-router-dom'
import React, { useEffect }  from 'react'
import Role from '../Components/Role'
import {
  connectMetaMaskWalletClick,
  connectMpcWalletClick,
  connectPrivateKeyWalletClick,
  disconnectWalletClick,
  updateContractState,
  updateInteractionVisibility,
} from "../WalletIntegration";

import {
  currentAccount, contractAddress
} from "../AppState"

import { isConnected } from '../AppState';

const Landing: React.FC = () => {

const navigate = useNavigate();
 
   useEffect(()=>{
  if(!isConnected){
    window.location.replace("/")
  }
   },[])

   const [file, setFile] = React.useState<File | null>(null);

  return (
    <>
  

    <div className='bg-grey-600 w-full p-2 text-2xl flex flex-col shadow-md'>
 <div >
How do you want to use the App?
 </div>
 <div className='flex flex-row gap-3 justify-center items-center mt-3'>
   <div onClick={()=> navigate("/doctor")}><Role role={"Doctor"}/></div>
   <div onClick={()=> navigate("/patient")}> <Role role={"Patient"}/></div>
   <div onClick={()=> navigate("/admin")}><Role role={"Admin"}/></div>
 </div>
    </div>

    </>
  )
}

export default Landing