// Import your Client Component
import Rooms from './Rooms';
import { headers } from 'next/headers';

async function getRooms() {
  const headersInstance = headers()
  const authorization = headersInstance.get('authorization')
  const res = await fetch('http://localhost:5000/rooms', { headers: { authorization } });
  const rooms = await res.json()
  return rooms;
}

export default async function Page() {
  // Fetch data directly in a Server Component
  const rooms = await getRooms();
  // Forward fetched data to your Client Component
  return <Rooms rooms={rooms} />;
}