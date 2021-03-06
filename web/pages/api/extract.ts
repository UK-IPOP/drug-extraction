// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next';
import path from 'path';
import fs from 'fs';

import { AlgorithmOutputDrug, AlgorithmOutputSimple } from '../../components/types';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
	if (req.method === 'POST') {
		const headers = Object.keys(req.body[0]);
		const csvString = [
			[headers.join(',')],
			...req.body.map((row: AlgorithmOutputSimple[] | AlgorithmOutputDrug[]) =>
				[...Object.values(row)].join(',')
			),
		].join('\n');
		res.send(csvString);
		return;
	}
	if (req.method === 'GET') {
		const filePath = path.join(process.cwd(), 'results.csv');
		const file = fs.readFileSync(filePath);
		res.setHeader('Content-Type', 'text/csv');
		res.setHeader('Content-Disposition', 'attachment; filename=results.csv');
		res.send(file);
		return;
	}
}
