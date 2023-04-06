

export default function Cases() {


  return (
    <>
      <div className="px-4 py-4 rounded-md bg-slate-400 w-full">
        <div>
          <h2 className="text-2xl text-center font-mono">Available Cases</h2>
        </div>
        <div className="flex flex-col gap-5">
          <div className="flex justify-between items-center px-4 py-4 bg-slate-500 rounded-sm">
            <p className="px-2 py-2 ">
              Image 1
            </p>
            <p className="px-2 py-2 ">
              Image 2
            </p>
            <p className="px-2 py-2 ">
              Path
            </p>
          </div>
          <div className="flex justify-between items-center px-4 py-4 bg-slate-500 rounded-sm">
            <p className="px-2 py-2 ">
              Image 1
            </p>
            <p className="px-2 py-2 ">
              Image 2
            </p>
            <p className="px-2 py-2 ">
              Path
            </p>
          </div>
        </div>
      </div>
    </>
  )
}
