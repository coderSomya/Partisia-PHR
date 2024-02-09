import React, { ReactFC } from 'react';

interface RoleProps {
  role: string;
}

const Role: ReactFC<RoleProps> = ({ role} : {role: string}) => {
  return <div key={role} className='flex justify-center items-center h-[100px] w-[300px] bg-blue-300 text-white hover:bg-blue-800 cursor-pointer'>
    {role}
  </div>;
};

export default Role;