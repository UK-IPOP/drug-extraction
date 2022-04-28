// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next';

import fs from 'fs';
import path from 'path';
import { AlgorithmOutputDrug, AlgorithmOutputSimple } from '../../components/types';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
	if (req.method !== 'POST') {
		res.status(405).json({ error: 'Method not allowed' });
		return;
	}
	const filePath = path.resolve(process.cwd(), 'results/extracted_drugs.csv');

	if ('drugName' in Object.keys(req.body[0])) {
		console.log('drugRelated');
	} else {
		console.log('drugNotRelated');
		const headers = Object.keys(req.body[0]);
		const csvString = [
			[headers.join(',')],
			...req.body.map((row: AlgorithmOutputSimple[] | AlgorithmOutputDrug[]) =>
				[...Object.values(row)].join(',')
			),
		].join('\n');
		const csvWrite = fs.promises.writeFile(filePath, csvString, { encoding: 'utf8' });
		await csvWrite;
	}
	res.status(201).json({ success: true, message: 'File written' });
}
