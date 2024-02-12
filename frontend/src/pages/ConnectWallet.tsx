import React, { useEffect } from 'react'
import { connectMetaMaskWalletClick, connectMpcWalletClick, disconnectWalletClick } from '../WalletIntegration';

const ConnectWallet = () => {


    useEffect(()=>{
        disconnectWalletClick()
    }, [])
  return (
    <div className="pure-g">
    <div className="pure-u-1-1">
      <h1>Public Health</h1>

      <div>
        <h2>Account</h2>
        <div id="connection-status">
          <p>Currently not logged in.</p>
        </div>

        <div id="private-key-connect">
          <form onSubmit={()=>{return false;}} className="pure-form">
            <input
              className="bg-blue-500 w-contain h-[70px] text-white text-2xl p-2 m-2 rounded-xl"
              id="private-key-connect-btn"
              type="submit"
              value="Login using private key" />
            <input id="private-key-connect" name="private-key-value" type="password" />
          </form>
        </div>

        <div id="wallet-connect">
          <form className="pure-form">
            <input
              className="bg-blue-500 w-contain h-[70px] text-white text-2xl p-2 m-2 rounded-xl hover:bg-blue-300 cursor-pointer"
              id="wallet-connect-btn"
              type="button"
              onClick={connectMpcWalletClick}
              value="Login using MPC Wallet" />
          </form>
        </div>

        <div id="metamask-connect">
          <form>
            <input
              className="bg-blue-500 w-contain h-[70px] text-white text-2xl p-2 m-2 rounded-xl hover:bg-blue-300 cursor-pointer"
              onClick={connectMetaMaskWalletClick}
              id="metamask-connect-btn"
              type="button"
              value="Login using MetaMask snap" />
          </form>
        </div>

        <div id="wallet-disconnect" className="hidden">
          <form>
            <input
              id="wallet-disconnect-btn"
              type="button"
              value="Logout"
              onClick={disconnectWalletClick} />
          </form>
        </div>
      </div>
    </div>
  </div>
  )
}

export default ConnectWallet