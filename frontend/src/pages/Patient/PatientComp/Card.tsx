// Card.tsx
import React from 'react';

interface CardProps {
  icon: string;
  title: string;
  info: string;
}

const Card: React.FC<CardProps> = ({ icon, title, info }) => {
  return (
    <div className="bg-green-500 rounded-lg shadow-md p-4 flex items-center justify-between hover:bg-green-600 transition duration-300 ease-in-out">
      <div className="text-white text-4xl">{icon}</div>
      <div className="ml-4">
        <h3 className="text-white text-lg font-bold">{title}</h3>
        <p className="text-white">{info}</p>
      </div>
    </div>
  );
}

export default Card;
