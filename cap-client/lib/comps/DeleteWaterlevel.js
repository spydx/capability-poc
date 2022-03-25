import { useState } from "react"
import { useGnapContext } from "../GnapProvider";

const DeleteWaterlevel = (bowl) => {

    const [id, setId ] = useState("");
    const {delete_waterlevel_data, delete_waterlevel} = useGnapContext();
    

    const btn = "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 border border-gray-400 rounded shadow"

    return (<>
        <div>
            <div className='flex flex-col item-center'>
                <p>Delete Waterlevel</p>
                <input 
                    type="text"
                    className="py-2 px-4"
                    value={id}
                    onChange={(e) => setId(e.target.value)}
                    placeholder="Bowl ID (integer)" 
                    />
                <button className={btn}
                    onClick={ () => {
                        if (id != "") {
                            console.log("clicky bitch")
                            delete_waterlevel(id)
                        }
                    }}
                >Delete</button>
            </div>
            <div>
                { delete_waterlevel_data == null ? <div></div> : <div>{delete_waterlevel_data.status}</div>}
            </div>
        </div>
    </>);
}

export default DeleteWaterlevel;