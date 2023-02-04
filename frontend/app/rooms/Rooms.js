'use client';

export default function Rooms({ rooms })  {
    return (
        <ul>
          {rooms.map((room) => (
            <li>{room.name}</li>
          ))}
        </ul>
      )
}