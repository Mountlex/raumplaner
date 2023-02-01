'use client';

export default function HomePage({ rooms })  {
    return (
        <ul>
          {rooms.map((room) => (
            <li>{room.name}</li>
          ))}
        </ul>
      )
}