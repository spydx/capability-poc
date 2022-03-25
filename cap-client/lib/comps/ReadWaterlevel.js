import { useState } from "react"
import { useGnapContext } from "../GnapProvider";

const ReadWaterlevel = (bowl) => {

    const [id, setId ] = useState("");
    const {readWaterlevel, get_waterlevel_by_id} = useGnapContext();
    

    const btn = "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 border border-gray-400 rounded shadow"

    return (<>
        <div>
            <div className='flex flex-col item-center'>
                <p>Waterlevel</p>
                <input 
                    type="text"
                    className="py-2 px-4"
                    value={id}
                    onChange={(e) => setId(e.target.value)}
                    placeholder="Waterlevel Id (integer)" 
                    />
                <button className={btn}
                    onClick={ () => {
                        if (id != "") {
                            console.log("clicky bitch")
                            get_waterlevel_by_id(id)
                        }
                    }}
                >Read</button>
            </div>
            <div>
                { readWaterlevel == null ? <div></div> : <div className="font-bold">Result: {readWaterlevel.id} {readWaterlevel.waterlevel} {readWaterlevel.date}</div>}
            </div>
        </div>
    </>);
}

export default ReadWaterlevel;