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
        <div className='px-2 text-2xl font-bold'>Bowl</div>
        <div className="grid grid-rows-3 grid-flow-col gap-4">
          
          <div className="row-span-3">
            <Menu />
          </div>
          <div className="col-span-2 bg-red-200">
           { requestMap == null ? <>RequestMap</> : <div>
              <ul>
                <il> Key: Value</il>
              </ul>
              <hr />
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
          <div className="row-span-2 col-span-2 bg-blue-200">
            03
            { showCreate ? <CreateBowl /> : ""}
            { showRead ? <ReadBowl /> : ""}
            </div>
        </div>
    </>
  )
}