"use client";

import { useEffect, useState } from "react";
// import Image from "next/image";

interface PersonData {
  name: string;
  age: number;
  city: string;
}

export default function Home() {

  const [person, setPerson] = useState<PersonData | null>(null);

  useEffect(() => {
    const fetchData = async () => {
    const response = await fetch("http://localhost:8000/blog/test");
    const data: PersonData = await response.json();
    setPerson(data);
  };

  fetchData();
  }, []);

  if (!person) return <p>Loading...</p>;

  return (
    <div>
      <h1>Fetched Data:</h1>
      <div>
        <h1>個人情報</h1>
        <p>名前: {person.name}</p>
        <p>年齢: {person.age}</p>
        <p>都市: {person.city}</p>
      </div>
    </div>
  );
}
