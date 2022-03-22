import { useRouter } from 'next/router';
import { useEffect } from 'react';
import { useGnapContext } from '../lib/GnapProvider';
import Menu from '../lib/menu';
import CreateBowl from '../lib/comps/CreateBowl';

export default function Home() {
  const router = useRouter();
  const {tx,  setTransaction, showCreate } = useGnapContext()

  
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
           02
          </div>
          <div className="row-span-2 col-span-2 bg-blue-200">
            03
            { showCreate ? <CreateBowl /> : ""}
            </div>
        </div>
    </>
  )
}