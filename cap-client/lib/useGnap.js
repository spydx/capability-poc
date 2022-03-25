import { useState } from "react";
import { v4 } from "uuid";

let uuid = v4();
const request = {
  "access_token": [
    {
      "label": "create_bowls",
      "access": [
        {
          "type": "waterbowl-access",
          "actions": [
            "create"
          ],
          "locations": ["http://localhost:8080/bowls/"]
        }
      ],
      "flags": [
        "bearer"
      ]
    },
    {
      "label": "read_bowls",
      "access": [
        {
          "type": "waterbowl-access",
          "actions": [
            "read"
          ],
          "locations": ["http://localhost:8080/bowls/"]
        }
      ],
      "flags": [
        "bearer"
      ]
    },
    {
      "label": "read_waterlevel",
      "access": [
        {
          "type": "waterlevel-access",
          "actions": [
            "read"
          ],
          "locations": ["http://localhost:8080/waterlevels/"]
        }
      ],
      "flags": [
        "bearer"
      ]
    },
    {
      "label": "create_waterlevel",
      "access": [
        {
          "type": "waterlevel-access",
          "actions": [
            "create"
          ],
          "locations": ["http://localhost:8080/waterlevels/"]
        }
      ],
      "flags": [
        "bearer"
      ]
    },
    {
      "label": "readll_waterlevel",
      "access": [
        {
          "type": "waterlevel-access",
          "actions": [
            "readall"
          ],
          "locations": ["http://localhost:8080/waterlevels/"]
        }
      ],
      "flags": [
        "bearer"
      ]
    },
    {
      "label": "delete_waterlevel",
      "access": [
        {
          "type": "waterlevel-access",
          "actions": [
            "delete"
          ],
          "locations": ["http://localhost:8080/waterlevels/"]
        }
      ],
      "flags": [
        "bearer"
      ]
    },
  ],
  "client": "7e057b0c-17e8-4ab4-9260-2b33f32b2cce",
  "interact": {
    "start": ["redirect"],
    "finish": {
      "method": "redirect",
      "uri": "localhost:8000/gnap/auth",
      "nonce": uuid.toString()
    }
  }
}


export default function useGnap() {
  const [user, setUser] = useState(null)
  const [redirect, setRedirect] = useState(false);
  const [tx, setTx] = useState(null);
  const [gnapCreateRequest, setGnapCreateRequest] = useState(null);
  const [gnapReadRequest, setGnapReadRequest] = useState(null);
  const [gnapDeleteRequest, setGnapDeleteRequest] = useState(null);
  const [gnapResponse, setGnapResponse] = useState(null);
  const [accessTokenMap, setAccessTokenMap] = useState(new Map());
  const [requestMap, setRequestMap] = useState(null);
  const [read_resourse_data, setReadResourseData] = useState(null);
  const [showCreate, setShowCreate] = useState(false);
  const [showRead, setShowRead] = useState(false);
 
  const [showUpdate, setShowUpdate] = useState(false);
  const [showDelete, setShowDelete] = useState(false);

  const [create_resourse_data, setCreateResourseData] = useState(null);
  const [create_waterlevel_data, setCreateWaterLevel ] = useState(null);

  const [showCreateWaterlevel, setShowCreateWaterlevel ] = useState()
  const [showReadWaterlevel, setShowReadWaterlevel ] = useState()
  const [showUpdateWaterlevel, setShowUpdateWaterlevel ] = useState()
  const [showDeleteWaterlevel, setShowDeleteWaterlevel ] = useState()

  const [readWaterlevel, setReadWaterlevel] = useState();

  const redirectLogin = (value) => {
    if (value) {
      setRedirect(value)
    }
  }


  const setTransaction = async (val) => {
    setTx(val)
  }
  const gnap_all_request = async () => {
    let data = await fetch("http://localhost:8000/gnap/tx",
      {
        method: "POST",
        headers: {
          "Content-type": "application/json"
        },
        body: JSON.stringify(request)
      }
    ).then(res => res.json())
      .then(d => d)
      .catch((err) => console.log(err))

    setGnapCreateRequest(data)
    console.log(data)
    createRequestMap(request)
    setTx(data.instance_id)
  }

  const createRequestMap = (data) => {
    const requstMap = new Map();
    let access_token = data.access_token;
    for (let token of access_token) {
      requstMap.set(token.label, token.access[0].locations[0])
    }

    setRequestMap(requstMap)
    console.log(requestMap)
  }

  const gnap_read_request = async () => {
    let read_request = {
      "access_token": {
        "access": [
          {
            "type": "waterbowl-access",
            "actions": [
              "read"
            ],
            "locations": ["http://localhost:8080/bowls/"]
          }
        ],
        "label": "bowls",
        "flags": [
          "bearer"
        ]
      },
      "client": "7e057b0c-17e8-4ab4-9260-2b33f32b2cce",
      "interact": {
        "start": ["redirect"],
        "finish": {
          "method": "redirect",
          "uri": "localhost:8000/gnap/auth",
          "nonce": uuid.toString()
        }
      }
    }
    let data = await fetch("http://localhost:8000/gnap/tx",
      {
        method: "POST",
        headers: {
          "Content-type": "application/json"
        },
        body: JSON.stringify(request)
      }
    ).then(res => res.json())
      .then(d => d)
      .catch((err) => console.log(err))

    setGnapReadRequest(data)
    console.log(data)
    setTx(data.instance_id)
  }

  const gnap_delete_request = async () => {
    let uuid = v4();
    let request = {
      "access_token": {
        "access": [
          {
            "type": "waterbowl-access",
            "actions": [
              "create"
            ],
            "locations": ["http://localhost:8080/bowls/"]
          }
        ],
        "label": "bowls",
        "flags": [
          "bearer"
        ]
      },
      "client": "7e057b0c-17e8-4ab4-9260-2b33f32b2cce",
      "interact": {
        "start": ["redirect"],
        "finish": {
          "method": "redirect",
          "uri": "localhost:8000/gnap/auth",
          "nonce": uuid.toString()
        }
      }
    }
    let data = await fetch("http://localhost:8000/gnap/tx",
      {
        method: "POST",
        headers: {
          "Content-type": "application/json"
        },
        body: JSON.stringify(request)
      }
    ).then(res => res.json())
      .then(d => d)
      .catch((err) => console.log(err))

    setGnapDeleteRequest(data)
    console.log(data)
    setTx(data.instance_id)
  }

  const gnap_contiuation = async () => {
    if (tx != null) {
      let contreq = { "interact_ref": tx };
      let url = "http://localhost:8000/gnap/tx/" + tx
      let response = await fetch(url,
        {
          method: "POST",
          headers: {
            "Content-type": "application/json"
          },
          body: JSON.stringify(contreq)
        }
      ).then(res => res.json())
        .then(d => d)
        .catch((err) => console.log(err))
      setGnapResponse(response)
      createTokenMap(response)
      console.log("GC Resp")
      console.log(response)
      console.log(accessTokenMap)
    }
  }

  const createTokenMap = (data) => {
    let access_token = data.access_token;
    const temp = new Map()
    for (let token of access_token) {
      temp.set(token.label, token.value)
    }
    setAccessTokenMap(temp)
    createRequestMap(request)
  }

  const create_resourse = async (name) => {
    if (accessTokenMap != null) {

      let url = requestMap.get("create_bowls");
      let token = accessTokenMap.get("create_bowls");
      let bowl = { "name": name };
      let status = await fetch(url,
        {
          method: "POST",
          headers: {
            "Content-type": "application/json",
            "Authorization": "Bearer " + token
          },
          body: JSON.stringify(bowl)
        }
      ).then(res => res.json())
      .then(d => d)
      .catch((err) => console.log(err))
      setCreateResourseData(status)
      console.log(status)
    }
  }

  const read_resource = async (id) => {
    if (accessTokenMap != null && requestMap != null) {

      let url = requestMap.get("read_bowls");
      let token = accessTokenMap.get("read_bowls");
      let data = await fetch(url + id,
        {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token
          }
        }
      ).then(res => {
        console.log(res);
        return res.json()})
      .then(d => d)
      .catch((err) => console.log(err))
      setReadResourseData(data)
      console.log(data)
    }
  }

  const delete_resource = async (id) => {
    if (accessTokenMap != null && requestMap != null) {

      let url = requestMap.get("delete_bowls");
      let token = accessTokenMap.get("delete_bowls");
      let data = await fetch(url + id,
        {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token
          }
        }
      ).then(res => {
        console.log(res);
        return res.json()})
      .then(d => d)
      .catch((err) => console.log(err))
      setReadResourseData(data)
      console.log(data)
    }
  }

  const get_waterlevel_by_id = async (id) => {
    if (accessTokenMap != null && requestMap != null) {
      let url = requestMap.get("read_waterlevel")
      let token = accessTokenMap.get("read_waterlevel")
      let data = await fetch(url + id, 
        {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token
          }
        })
        .then(res => res.json())
        .then(d => d)
        .catch(err => console.log(err))
        setReadWaterlevel(data)
    }
  }

  const create_waterlevel = async (bowlid, waterlevel) => {
    if (accessTokenMap != null && requestMap != null) {
      let post_data = { "waterlevel": Number(waterlevel)};
      let url = requestMap.get("create_waterlevel")
      let token = accessTokenMap.get("create_waterlevel")
      let response = await fetch(url + bowlid, {
        method: "POST",
        headers: {
          
          "Content-Type": "application/json",
          "Authorization": "Bearer " + token
        },
        body: JSON.stringify(post_data)
      })
      .then(res => res.json())
      .then(d => d)
      .catch(err => console.log(err))
      setCreateWaterLevel(response)
    }
  }
  

  return {
    user,
    redirect,
    redirectLogin,
    tx,
    setTransaction,
    gnapCreateRequest,
    gnapReadRequest,
    gnapDeleteRequest,
    gnap_all_request,
    gnap_contiuation,
    showCreate,
    setShowCreate,
    create_resourse,
    gnapResponse,
    setRequestMap,
    requestMap,
    setAccessTokenMap,
    accessTokenMap,
    setShowRead,
    showRead,
    read_resource,
    read_resourse_data, 
    setReadResourseData,
    create_resourse_data, setCreateResourseData,
    showUpdate, setShowUpdate,
    showDelete, setShowDelete,
    showCreateWaterlevel, setShowCreateWaterlevel,
    showReadWaterlevel, setShowReadWaterlevel,
    showUpdateWaterlevel, setShowUpdateWaterlevel,
    showDeleteWaterlevel, setShowDeleteWaterlevel,
    readWaterlevel, setReadWaterlevel,
    get_waterlevel_by_id,
    create_waterlevel_data, create_waterlevel
  }
}