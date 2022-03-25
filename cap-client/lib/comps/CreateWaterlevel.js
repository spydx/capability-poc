import { useState } from "react"
import { useGnapContext } from "../GnapProvider";

const CreateWaterlevel = (bowl) => {

    const [id, setId ] = useState("");
    const [waterlevel, setWaterlevel] = useState("");
    const {create_waterlevel_data, create_waterlevel} = useGnapContext();
    

    const btn = "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 border border-gray-400 rounded shadow"

    return (<>
        <div>
            <div className='flex flex-col item-center'>
                <p>Create Waterlevel</p>
                <input 
                    type="text"
                    className="py-2 px-4"
                    value={id}
                    onChange={(e) => setId(e.target.value)}
                    placeholder="Bowl (integer)" 
                    />
                <input 
                    type="text"
                    className="py-2 px-4"
                    value={waterlevel}
                    onChange={(e) => setWaterlevel(e.target.value)}
                    placeholder="Waterlevel (integer)" 
                    />
                <button className={btn}
                    onClick={ () => {
                        if (id != "" && waterlevel != "") {
                            console.log("clicky bitch")
                            create_waterlevel(id, waterlevel)
                        }
                    }}
                >Create</button>
            </div>
            <div>
                { create_waterlevel_data == null ? <></> : <div className="font-bold">Created: {create_waterlevel_data.id} {create_waterlevel_data.waterlevel} {create_waterlevel_data.date}</div>}
            </div>
        </div>
    </>);
}

export default CreateWaterlevel;