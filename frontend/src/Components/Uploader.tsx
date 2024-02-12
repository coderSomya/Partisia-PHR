import React, { useState } from 'react';
import PinataSDK  from "@pinata/sdk";


interface UploadOptions {
  pinToIPFS?: boolean;
  customPinCid?: string;
  metadata?: any;
}

const apiKey = "f2d297d932a3238e6b82630a28ff3e311fbf69b68adab570eb3e6d8b6b9a1aca"; 
const JWT = 
"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiI0OWM0YjczMy03NTUxLTRlMzMtOGYxZi1mMGYyY2VlY2JhN2YiLCJlbWFpbCI6Imlzb215YTEzQGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaW5fcG9saWN5Ijp7InJlZ2lvbnMiOlt7ImlkIjoiRlJBMSIsImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxfSx7ImlkIjoiTllDMSIsImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxfV0sInZlcnNpb24iOjF9LCJtZmFfZW5hYmxlZCI6ZmFsc2UsInN0YXR1cyI6IkFDVElWRSJ9LCJhdXRoZW50aWNhdGlvblR5cGUiOiJzY29wZWRLZXkiLCJzY29wZWRLZXlLZXkiOiI3NTdkN2ZhNzZhYTIzMmQ5ZWI0OCIsInNjb3BlZEtleVNlY3JldCI6ImYyZDI5N2Q5MzJhMzIzOGU2YjgyNjMwYTI4ZmYzZTMxMWZiZjY5YjY4YWRhYjU3MGViM2U2ZDhiNmI5YTFhY2EiLCJpYXQiOjE3MDc3NjAyOTB9.fRcIZsh7VWSF6BmS69NVwJ1kqLm2T-rcBP8Jd9dDmmI"

import axios from "axios"; 


// Upload File to Pinata
export const uploadFile = async (file: File): Promise<string> => {
  const formData = new FormData();
  formData.append("file", file);

  const pinataMetadata = JSON.stringify({
    name: "File name",
  });
  formData.append("pinataMetadata", pinataMetadata);

  const pinataOptions = JSON.stringify({
    cidVersion: 0,
  });
  formData.append("pinataOptions", pinataOptions);

  const resFile = await axios({
    method: "post",
    url: "https://api.pinata.cloud/pinning/pinFileToIPFS",
    data: formData,
    headers: {
      Authorization: `Bearer ${JWT}`,
      "Content-Type": "multipart/form-data",
    },
  });
  return `https://gateway.pinata.cloud/ipfs/${resFile.data.IpfsHash}`;
};


function Uploader() {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [uploadStatus, setUploadStatus] = useState<string>('');

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if(event.target && event.target.files) setSelectedFile(event.target.files[0]);
  };

  const handleUpload = async () => {
    if (!selectedFile) {
      setUploadStatus('Please select a file to upload.');
      return;
    }

    try {
      setUploadStatus('Uploading...');
      const url = await uploadFile(selectedFile);
      setUploadStatus(`File uploaded successfully! URL: ${url}`);
    } catch (error: any) {
      setUploadStatus(`Error uploading file: ${error.message}`);
    }
  };

  return (
    <div>
      <h1>Upload File to Pinata</h1>
      <input type="file" onChange={handleFileChange} />
      <button onClick={handleUpload}>Upload</button>
      <p>{uploadStatus}</p>
    </div>
  );
}

export default Uploader;
