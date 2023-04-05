import Link from 'next/link'

function Routes() {
  return (
    <ul className='flex justify-between gap-5 px-4 py-4 items-center bg-gray-800 mx-auto rounded-md'>
  <li className='px-2 py-2 bg-gray-800 hover:bg-gray-600'>
        <Link href="/">Home</Link>
      </li>
      <li className='px-2 py-2 bg-gray-800 hover:bg-gray-600'>
        <Link href="/timeline">Timeline</Link>
      </li>
      <li className='px-2 py-2 bg-gray-800 hover:bg-gray-600'>
        <Link href="/files">Files</Link>
      </li>
    </ul>
  )
}

export default Routes
