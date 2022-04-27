// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next';

import fs from 'fs';
import path from 'path';

export default function handler(_: NextApiRequest, res: NextApiResponse) {
	res.setHeader('Content-Type', 'text/csv');
	res.setHeader('Content-Disposition', 'attachment; filename=extracted_drugs.csv');
	const filePath = path.resolve(process.cwd(), 'public/results/extracted_drugs.csv');
	const fileBuffer = fs.readFileSync(filePath, { encoding: 'utf8' });
	res.send(fileBuffer);
}
