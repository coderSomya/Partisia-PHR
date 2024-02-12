import "./App.css";
import Doctor from "./pages/Doctor/Doctor";
import Landing from "./pages/Landing";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Patient from "./pages/Patient/Patient";
import Admin from "./pages/Admin/Admin";
import PatientData from "./pages/Admin/PatientData";
import DoctorData from "./pages/Admin/DoctorData";

function App() {
  return (
    <>
      <Router>
        <Routes>
          <Route path="/" element={<Landing />} />
          <Route path="/doctor" element={<Doctor />} />
          <Route path="/patient" element={<Patient />} />
          <Route path="/admin" element={<Admin />} />
          <Route path="/admin/doctor-data" element={<DoctorData />} />
          <Route path="/admin/patient-data" element={<PatientData />} />
        </Routes>
      </Router>
    </>
  );
}

export default App;
