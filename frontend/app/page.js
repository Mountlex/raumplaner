// Import your Client Component
import HomePage from './HomePage';

async function getRooms() {
  const res = await fetch('http://localhost:5000/rooms');
  const rooms = await res.json()
  console.log(rooms)
  return rooms;
}

export default async function Page() {
  // Fetch data directly in a Server Component
  const rooms = await getRooms();
  // Forward fetched data to your Client Component
  return <HomePage rooms={rooms} />;
}