import { useEffect, useState } from "react";
import { useGnapContext } from "./GnapProvider";

export default function Menu() {
    const { gnap_request, gnap_contiuation, tx, redirect, redirectLogin } = useGnapContext()
    

    const btn = "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 border border-gray-400 rounded shadow"


    useEffect(() => {
      if(redirect && tx !== null) { 
        let buffer = Buffer.from("http://localhost:3000/", 'utf8')
        const client = buffer.toString('base64');
        let url = "http://localhost:8000/login/?tx=" + tx + "&client=" + client
        window.location.href = url
      } else {
        console.log("Create a GnapRequest First")
      }
    },[redirect])

    console.log("TX: " + tx)
    return (
        <>
        <div className="flex flex-col">
          <div className="p-2"> 
            <button className={btn}
              onClick={() => gnap_request()}
            >1. Gnap Request</button>
          </div>
          <div className="p-2">
            <button className={btn}
            onClick={() => {
              redirectLogin(true)
              }}
            >2. Login</button> 
          </div>
          <div className="p-2">
            <button className={btn}
              onClick={() => gnap_contiuation()}>
              3. Continuation Request
            </button>
          </div>
          <div className="">
            <div className="p-2">
              <div>Actions:</div>
            </div>
            <div className="p-2">
              <button className={btn}>Create a resource</button> 
            </div>
            <div className="p-2">
              <button className={btn}
              >Read a resource</button> 
            </div>
            <div className="p-2">
              <button className={btn}>Delete a resource</button> 
            </div>
          </div>
        </div>
        </>
    )
}