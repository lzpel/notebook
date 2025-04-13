'use client'

import React, { useState } from 'react'
import {output_diary} from "@/output";

export default function Home() {
    const [startDate, setStartDate] = useState('')
    const [days, setDays] = useState(30)

    const handleGenerate = async () => {
        const data = output_diary()

        const blob = new Blob([data], { type: 'application/pdf' })
        const url = URL.createObjectURL(blob)

        const link = document.createElement('a')
        link.href = url
        link.download = `diary_${startDate}_${days}days.pdf`
        link.click()

        URL.revokeObjectURL(url)
    }

    return (
        <div className="p-4 space-y-4">
            <h1 className="text-xl font-bold">手帳PDFの生成</h1>

            <div>
                <label className="block font-medium">開始日（YYYY-MM-DD）</label>
                <input
                    type="date"
                    value={startDate}
                    onChange={(e) => setStartDate(e.target.value)}
                    className="border rounded px-2 py-1"
                />
            </div>

            <div>
                <label className="block font-medium">日数</label>
                <input
                    type="number"
                    value={days}
                    onChange={(e) => setDays(Number(e.target.value))}
                    min={1}
                    className="border rounded px-2 py-1"
                />
            </div>

            <button
                onClick={handleGenerate}
                className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
            >
                PDFを生成
            </button>
        </div>
    )
}