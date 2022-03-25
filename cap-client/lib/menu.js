import { useEffect } from "react";
import { useGnapContext } from "./GnapProvider";

export default function Menu() {
    const { accessTokenMap, gnap_all_request, gnap_contiuation, tx, } = useGnapContext()
    const { redirect, redirectLogin} = useGnapContext()
    const { showCreate, setShowCreate }= useGnapContext()
    const { showRead, setShowRead} = useGnapContext()
    const { showCreateWaterlevel, setShowCreateWaterlevel } = useGnapContext()
    const { showReadWaterlevel, setShowReadWaterlevel} = useGnapContext()
    const { showUpdateWaterlevel, setShowUpdateWaterlevel } = useGnapContext()
    const { showDeleteWaterlevel, setShowDeleteWaterlevel } = useGnapContext()
  
    console.log(tx)
    const bluebtn = "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 border border-gray-400 rounded shadow"
    const redbtn = "bg-red-500 hover:bg-red-700 text-white py-2 px-4 border border-gray-400 rounded shadow"


    useEffect(() => {
      if(redirect && tx !== null) { 
        let buffer = Buffer.from("http://localhost:3000/", 'utf8')
        const client = buffer.toString('base64');
        let url = "http://localhost:8000/login/?tx=" + tx + "&client=" + client
        window.location.href = url
      } else {
        console.log("Create a GnapRequest First")
      }
    },[redirect, tx])

    return (
        <>
        <div className="flex flex-col">
          <div className="p-1"> 
            <button className={bluebtn}
              onClick={() => gnap_all_request()}
            >1. Gnap Request</button>
          </div>
          <div className="p-1">
            <button className={ tx != null ? bluebtn : redbtn}
            onClick={() => {
              redirectLogin(true)
              }}
            >2. Login</button> 
          </div>
          <div className="p-1">
            <button className={ tx != null ? bluebtn : redbtn}
              onClick={() => gnap_contiuation()}>
              3. Continuation Request
            </button>
          </div>
          <div className="">
            <div className="p-1">
              <div className="font-bold">Actions:</div>
            </div>
            <div className="p-1">
              <button className={accessTokenMap.has("create_bowls") ? bluebtn: redbtn}
                onClick={() => {
                  if(showCreate) {
                    setShowCreate(false) 
                  } else {
                    setShowCreate(true)
                  }
                }}
              >Create a Bowl</button> 
            </div>
            <div className="p-1">
              <button className={accessTokenMap.has("read_bowls") ? bluebtn : redbtn}
              onClick={() => {
                if(showRead) {
                  setShowRead(false) 
                } else {
                  setShowRead(true)
                }
              }}
              >Read a Bowl</button> 
            </div>

            <div className="p-1">
              <button className={accessTokenMap.has("create_waterlevel") ? bluebtn : redbtn}
                onClick= {() => {
                  if(showCreateWaterlevel) {
                    setShowCreateWaterlevel(false)
                  } else {
                    setShowCreateWaterlevel(true)
                  }
                }}
              >Create a Waterlevel</button> 
            </div>
            <div className="p-1">
              <button className={accessTokenMap.has("read_waterlevel") ? bluebtn : redbtn}
               onClick= {() => {
                if(showReadWaterlevel) {
                  setShowReadWaterlevel(false)
                } else {
                  setShowReadWaterlevel(true)
                }
              }}
              >Read a Waterlevel</button> 
            </div>
            <div className="p-1">
              <button className={accessTokenMap.has("delete_waterlevel") ? bluebtn : redbtn}
              onClick={() => {
                if(showDeleteWaterlevel) {
                  setShowDeleteWaterlevel(false)
                } else {
                  setShowDeleteWaterlevel(true)
                }
              }}
              >Delete a Waterlevel</button> 
            </div>
            
          </div>
        </div>
        </>
    )
}