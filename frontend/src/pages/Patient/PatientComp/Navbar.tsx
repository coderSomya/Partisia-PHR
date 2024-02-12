import React, { useState } from 'react';

const Navbar = () => {
  const [contactDropdownOpen, setContactDropdownOpen] = useState(false);
  const [otherDropdownOpen, setOtherDropdownOpen] = useState(false);

  const toggleContactDropdown = () => {
    setContactDropdownOpen(!contactDropdownOpen);
  };

  const toggleOtherDropdown = () => {
    setOtherDropdownOpen(!otherDropdownOpen);
  };

  return (
    <><nav className="bg-green-800">
      <div className="container mx-auto px-5 py-3 flex justify-between items-center">
        <button className="text-white text-xl border border-white rounded-lg px-3 py-2 inline-block lg:hidden hover:bg-green-600 focus:outline-none">
          <span className="block">&#9776;</span>
        </button>
        <div className="hidden lg:flex items-center space-x-8">
          <a className="text-white hover:text-green-300 text-lg" href="#">Home</a>
          <div className="relative">
            <a className="text-white hover:text-green-300 text-lg" href="#" role="button" onClick={toggleContactDropdown}>
              Contact Us
            </a>
            {contactDropdownOpen && (
              <div className="dropdown-menu absolute bg-white text-gray-800 rounded-lg shadow-lg right-0 mt-2">
                <ul>
                  <li><a className="block py-2 px-4 text-sm hover:bg-green-100" href="#">Email</a></li>
                  <li><a className="block py-2 px-4 text-sm hover:bg-green-100" href="#">Phone</a></li>
                  {/* Add more options as needed */}
                </ul>
              </div>
            )}
          </div>
          <div className="relative">
            <a className="text-white hover:text-green-300 text-lg" href="#" role="button" onClick={toggleOtherDropdown}>
              Dropdown
            </a>
            {otherDropdownOpen && (
              <div className="dropdown-menu absolute bg-white text-gray-800 rounded-lg shadow-lg right-0 mt-2">
                <ul>
                  <li><a className="block py-2 px-4 text-sm hover:bg-green-100" href="#">Action</a></li>
                  <li><a className="block py-2 px-4 text-sm hover:bg-green-100" href="#">Another action</a></li>
                  <li><hr className="border-gray-200" /></li>
                  <li><a className="block py-2 px-4 text-sm hover:bg-green-100" href="#">Something else here</a></li>
                </ul>
              </div>
            )}
          </div>
          <a className="text-white hover:text-green-300 text-lg mr-4" href="#" aria-disabled="true">Disabled</a>
        </div>
        
      </div>
      
    </nav>
    <form className="flex items-center ml-20">
    <input className="form-input rounded-md border-none bg-green-100 py-1 px-2 focus:outline-none text-black" type="search" placeholder="Search" aria-label="Search" />
    <button className="btn bg-green-600 text-white py-1 px-4 rounded-md hover:bg-green-700 focus:outline-none" type="submit">Search</button>
  </form></>
  );
}

export default Navbar;
