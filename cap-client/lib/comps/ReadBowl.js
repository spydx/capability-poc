import { useState } from "react"
import { useGnapContext } from "../GnapProvider";

const ReadBowl = (bowl) => {

    const [id, setId ] = useState("");
    const {read_resourse_data, read_resource} = useGnapContext();
    

    const btn = "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 border border-gray-400 rounded shadow"

    return (<>
        <div>
            <div className='flex flex-col item-center'>
                <p>Read Bowl</p>
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
                            read_resource(id)
                        }
                    }}
                >Read</button>
            </div>
            <div>
                { read_resourse_data == null ? <div></div> : <div className="font-bold">Result: {read_resourse_data.id} : {read_resourse_data.name}</div>}
            </div>
        </div>
    </>);
}

export default ReadBowl;