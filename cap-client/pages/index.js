import { useRouter } from 'next/router';
import { useEffect } from 'react';
import { useGnapContext } from '../lib/GnapProvider';
import Menu from '../lib/menu';
import CreateBowl from '../lib/comps/CreateBowl';
import ReadBowl from '../lib/comps/ReadBowl';

export default function Home() {
  const router = useRouter();
  const {tx, showRead,  setTransaction, showCreate, requestMap, accessTokenMap } = useGnapContext()

  
  useEffect( () => {
    if (router.query.tx != null) {
      setTransaction(router.query.tx)
    }
  }, [router.query.tx, setTransaction, tx])

  return (
    <>
        <div className='px-4 text-2xl font-bold'>Bowl Managment</div>
        <div className="grid grid-rows-3 grid-flow-col gap-4 px-2">
          
          <div className="row-span-3">
            <Menu />
          </div>
          <div className="col-span-2 bg-red-200 p-2">
           { requestMap == null ? <>RequestMap</> : <div>
             <div className='font-bold'>
              <ul>
                <il> Label : Path</il>
              </ul>
              <hr />
              </div>
             { 
             Array.from(requestMap, function(item) {
               return (<>
                <ul>
                  <il>{item[0]} : {item[1]}</il>
                </ul>
                <hr />
                </>);
             })
             }</div>}
           { accessTokenMap == null ? <p>TokenMap</p> : <>
           <div className='font-bold'>
              <ul>
                <il> Access : Token </il>
              </ul>
              <hr />
              </div>
           {
             Array.from(accessTokenMap, function(access) {
              return (<>
                <ul>
                  <il>{access[0]} : {access[1]}</il>
                </ul>
                <hr />
                </>);
             })
           }</>}

          </div>
          <div className="row-span-2 col-span-2 bg-blue-200 p-2">
              <div className='font-bold'>
                <ul>
                  <il>Action and Data</il>
                </ul>
                <hr />
              </div>
            { showCreate ? <CreateBowl /> : ""}
            { showRead ? <ReadBowl /> : ""}
            </div>
        </div>
    </>
  )
}