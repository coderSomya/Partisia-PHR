import "./App.css";
import Doctor from "./pages/Doctor/Doctor";
import Landing from "./pages/Landing";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Patient from "./pages/Patient/Patient";
import Admin from "./pages/Admin/Admin";
import PatientData from "./pages/Admin/PatientData";
import DoctorData from "./pages/Admin/DoctorData";
import ConnectWallet from "./pages/ConnectWallet";


function App() {

  return (
    <>
    <Router>
      <Routes>
        <Route path="/" element={<ConnectWallet/>}/>
        <Route path="/landing" element = { <Landing/> }/>
        <Route path="/doctor" element={<Doctor/>}/>
        <Route path="/patient" element={<Patient/>}/>
      </Routes>
    </Router>
    </>
  );
}

export default App;
