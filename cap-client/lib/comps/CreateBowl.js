import { useState } from "react"
import { useGnapContext } from "../GnapProvider";

const CreateBowl = (bowl) => {

    const [name, setName ] = useState("");
    const {create_resourse, create_resourse_data} = useGnapContext();

    const btn = "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 border border-gray-400 rounded shadow"

    return (<>
        <div>
            <div className='flex flex-col item-center'>
                <p>Bowl Name</p>
                <input 
                    type="text"
                    className="py-2 px-4"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    placeholder="Bowl Name" 
                    />
                <button className={btn}
                    onClick={ () => {
                        if (name != "") {
                            console.log("clicky bitch")
                            create_resourse(name)
                        }
                    }}
                >Create</button>
            </div>
            <div>
                { create_resourse_data == null ? <div></div> : <div>Created: {create_resourse_data.status}</div>}
            </div>
        </div>
    </>);
}

export default CreateBowl;