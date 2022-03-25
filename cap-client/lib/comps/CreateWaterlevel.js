import { useState } from "react"
import { useGnapContext } from "../GnapProvider";

const CreateWaterlevel = (bowl) => {

    const [id, setId ] = useState("");
    const [waterlevel, setWaterlevel] = useState();
    const {delete_resourse_data, delete_resource} = useGnapContext();
    

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
                    value={id}
                    onChange={(e) => setWaterlevel(e.target.value)}
                    placeholder="Waterlevel (integer)" 
                    />
                <button className={btn}
                    onClick={ () => {
                        if (id != "") {
                            console.log("clicky bitch")
                        }
                    }}
                >Create</button>
            </div>
            <div>
                
            </div>
        </div>
    </>);
}

export default CreateWaterlevel;